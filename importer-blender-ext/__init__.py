from collections import namedtuple
import bpy
from pathlib import Path
from itertools import zip_longest

from typing import TYPE_CHECKING, Iterable, NamedTuple, Optional, assert_never

from bpy.types import (
    Panel,
    PropertyGroup,
)
from bpy.props import (
    BoolProperty,
    CollectionProperty,
    FloatProperty,
    IntProperty,
    PointerProperty,
    StringProperty,
)
import json

from . import directordata
from .directordata import DirectorData

if TYPE_CHECKING:
    from bpy.stub_internal.rna_enums import OperatorReturnItems


class ActuatorProperties(PropertyGroup):
    name: StringProperty(default="")
    prepare_time: FloatProperty(name="Prepare Time", default=0.1, unit="TIME")
    release_time: FloatProperty(name="Release Time", default=0.2, unit="TIME")
    release_offset: FloatProperty(name="Release Offset", default=0, unit="TIME")
    interpolate: BoolProperty(name="Interpolate Keyframes", default=True)


class Music3DAnimationProperties(PropertyGroup):
    director_data_path: StringProperty(
        name="Director Data File",
        description="Music3DAnimation Director Data",
        default="",
        maxlen=1024,
        subtype="FILE_PATH",
    )
    actuators: CollectionProperty(
        type=ActuatorProperties,
        name="Actuators",
    )
    selected_actuator: IntProperty()


class Music3DAnimationGeneratorPanel(Panel):
    bl_idname = "VIEW3D_PT_music3danimation"
    bl_label = "Music3DAnimation"
    bl_category = "Tool"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"

    @classmethod
    def poll(cls, context):
        return context.object is not None

    def draw_header(self, context):
        layout = self.layout
        assert layout is not None
        layout.label(text="Music3DAnimation")

    def draw(self, context):
        layout = self.layout
        assert layout is not None
        scene = context.scene
        assert scene is not None

        props: Music3DAnimationProperties = scene.music3danimation

        col = layout.column(align=True)
        col.prop(props, "director_data_path", text="")
        col.operator("object.import_music3danimation_director_file", text="Import")
        col.template_list(
            "VIEW3D_UL_music3danimation_list",
            "",
            props,
            "actuators",
            props,
            "selected_actuator",
        )
        col.operator("object.generate_music3danimation_keyframes", text="Generate Keyframes")
        # col.template_list


class Music3DAnimationActuatorList(bpy.types.UIList):
    bl_idname = "VIEW3D_UL_music3danimation_list"

    def draw_item(
        self,
        context,
        layout,
        data,
        item,
        icon,
        active_data,
        active_property,
        index,
        flt_flag,
    ) -> None:
        scene = context.scene
        assert scene is not None

        props: Music3DAnimationProperties = scene.music3danimation

        if self.layout_type in {"DEFAULT", "COMPACT"}:
            col = layout.column()
            col.label(text=item.name)
            col.prop(item, "prepare_time")
            col.prop(item, "release_time")
            col.prop(item, "release_offset")
            col.prop(item, "interpolate")


class ActuatorKeyFrame(NamedTuple):
    frame: int
    action: bool
    track: int
    note: int


director_data_text_block_name = "director_data"


class Music3DAnimationImport(bpy.types.Operator):
    bl_idname = "object.import_music3danimation_director_file"
    bl_label = "Import Music3DAnimation Director File"
    bl_description = "Import Music3DAnimation Director File"

    def execute(self, context) -> set["OperatorReturnItems"]:
        scene = context.scene
        assert scene is not None

        props: Music3DAnimationProperties = scene.music3danimation
        print(props.director_data_path)
        director_data_path = Path(bpy.path.abspath(props.director_data_path))

        assert director_data_path.is_file()

        if director_data_text_block_name in bpy.data.texts:
            text_block = bpy.data.texts[director_data_text_block_name]
            text_block.clear()
        else:
            text_block = bpy.data.texts.new(name=director_data_text_block_name)

        with open(director_data_path) as director_data_file:
            text_block.write(director_data_file.read())

        props.actuators.clear()

        data_string = bpy.data.texts[director_data_text_block_name].as_string()
        director_data = DirectorData.from_json(json.loads(data_string))

        for actuator in director_data.actuators:
            prop = props.actuators.add()
            prop.name = actuator.name

        return {"FINISHED"}


class Music3DAnimationGenerate(bpy.types.Operator):
    bl_idname = "object.generate_music3danimation_keyframes"
    bl_label = "Generate Music3DAnimation Keyframes"
    bl_description = "Generate Music3DAnimation keyframes from imported director file"

    def execute(self, context) -> set["OperatorReturnItems"]:
        scene = context.scene
        assert scene is not None

        props: Music3DAnimationProperties = scene.music3danimation

        if director_data_text_block_name not in bpy.data.texts:
            return {"CANCELLED"}

        data_string = bpy.data.texts[director_data_text_block_name].as_string()
        director_data = DirectorData.from_json(json.loads(data_string))

        print(director_data)

        actuators_collection = bpy.data.collections.get("actuators")
        if actuators_collection is None:
            actuators_collection = bpy.data.collections.new(name="actuators")
            scene.collection.children.link(actuators_collection)
        assert actuators_collection is not None

        fps = scene.render.fps
        for actuator in director_data.actuators:
            obj = actuators_collection.objects.get(actuator.name)
            if obj is None:
                obj = bpy.data.objects.new(name=actuator.name, object_data=None)
                actuators_collection.objects.link(obj)
            assert obj is not None

            obj.animation_data_clear()

            frame = 0
            previous_hit = frame

            actuator_props = props.actuators[actuator.name]
            prepare_time = actuator_props.prepare_time
            release_time = actuator_props.release_time
            release_offset = actuator_props.release_offset
            interpolate = actuator_props.interpolate

            frame += release_offset * fps

            def add_keyframe(kf: ActuatorKeyFrame):
                obj["action"] = float(kf.action) if interpolate else kf.action
                obj["track"] = float(kf.track) if interpolate else kf.track
                obj["note"] = float(kf.note) if interpolate else kf.note
                obj.keyframe_insert('["action"]', frame=kf.frame)
                obj.keyframe_insert('["track"]', frame=kf.frame)
                obj.keyframe_insert('["note"]', frame=kf.frame)

            first = True
            for note, next_note in zip_longest(actuator.notes, actuator.notes[1:]):
                frame += note.delta * fps
                end_preparation = max(frame - prepare_time * fps, previous_hit)
                complete_animation = frame + release_time * fps
                if next_note is not None:
                    next_frame = frame + next_note.delta * fps - prepare_time * fps
                    complete_animation = min(complete_animation, next_frame)
                if first:
                    add_keyframe(
                        ActuatorKeyFrame(
                            scene.frame_start, False, note.track, note.note
                        )
                    )
                    first = False
                add_keyframe(
                    ActuatorKeyFrame(int(end_preparation), False, note.track, note.note)
                )
                add_keyframe(ActuatorKeyFrame(int(frame), True, note.track, note.note))
                add_keyframe(
                    ActuatorKeyFrame(
                        int(complete_animation), False, note.track, note.note
                    )
                )
                animation_data = obj.animation_data
                assert animation_data is not None
                for layer in animation_data.action.layers:
                    for strip in layer.strips:
                        for channelbag in strip.channelbags:
                            for fcurve in channelbag.fcurves:
                                for pt in fcurve.keyframe_points:
                                    pt.interpolation = (
                                        "BOUNCE" if interpolate else "CONSTANT"
                                    )
                previous_hit = complete_animation

        return {"FINISHED"}


classes = [
    ActuatorProperties,
    Music3DAnimationProperties,
    Music3DAnimationActuatorList,
    Music3DAnimationGeneratorPanel,
    Music3DAnimationImport,
    Music3DAnimationGenerate,
]


def register():
    for _class in classes:
        bpy.utils.register_class(_class)
    bpy.types.Scene.music3danimation = PointerProperty(type=Music3DAnimationProperties)


def unregister():
    del bpy.types.Scene.music3danimation
    for _class in classes:
        bpy.utils.unregister_class(_class)


# This allows you to run the script directly from Blender's Text editor
# to test the add-on without having to install it.
if __name__ == "__main__":
    register()
