from collections import namedtuple
import bpy
from pathlib import Path

from typing import TYPE_CHECKING, Iterable, NamedTuple, Optional, assert_never

from bpy.types import (
    Panel,
    PropertyGroup,
)
from bpy.props import PointerProperty, StringProperty
import json

from . import demo1, directordata
from .directordata import DirectorData

if TYPE_CHECKING:
    from bpy.stub_internal.rna_enums import OperatorReturnItems


class AniMusicProperties(PropertyGroup):
    director_data: StringProperty(
        name="Director Data",
        description="Animusic Director Data",
        default="",
        maxlen=1024,
        subtype="FILE_PATH",
    )


class AnimusicGeneratorPanel(Panel):
    bl_idname = "VIEW3D_PT_animusic"
    bl_label = "Animusic"
    bl_category = "Tool"
    bl_space_type = "VIEW_3D"
    bl_region_type = "UI"

    @classmethod
    def poll(cls, context):
        return context.object is not None

    def draw_header(self, context):
        layout = self.layout
        assert layout is not None
        layout.label(text="Animusic")

    def draw(self, context):
        layout = self.layout
        assert layout is not None
        scene = context.scene
        assert scene is not None

        col = layout.column(align=True)
        col.prop(scene.animusic, "director_data", text="")
        col.operator("object.generate_animusic_keyframes", text="Import")


class ActuatorKeyFrame(NamedTuple):
    frame: int
    action: bool
    track: int
    note: int


class AnimusicGenerate(bpy.types.Operator):
    bl_idname = "object.generate_animusic_keyframes"
    bl_label = "Generate Animusic Keyframes"
    bl_description = "Generate Animusic keyframes from imported director file"

    def execute(self, context) -> set["OperatorReturnItems"]:
        scene = context.scene
        assert scene is not None

        props: AniMusicProperties = scene.animusic
        print(props.director_data)
        director_data_path = Path(
            bpy.path.abspath(props.director_data)
        )

        assert director_data_path.exists()

        with open(director_data_path) as director_data_file:
            director_data = DirectorData.from_json(json.load(director_data_file))

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

            def add_keyframe(kf: ActuatorKeyFrame):
                obj["action"] = kf.action
                obj["track"] = kf.track
                obj["note"] = kf.note
                obj.keyframe_insert('["action"]', frame=kf.frame)
                obj.keyframe_insert('["track"]', frame=kf.frame)
                obj.keyframe_insert('["note"]', frame=kf.frame)

            for note in actuator.notes:
                frame += note.delta * fps
                end_preparation = max(frame - demo1.prepare_time * fps, previous_hit)
                complete_animation = frame + demo1.release_time * fps
                add_keyframe(
                    ActuatorKeyFrame(int(end_preparation), False, note.track, note.note)
                )
                add_keyframe(ActuatorKeyFrame(int(frame), True, note.track, note.note))
                add_keyframe(
                    ActuatorKeyFrame(
                        int(complete_animation), False, note.track, note.note
                    )
                )
                previous_hit = complete_animation

        return {"FINISHED"}


def register():
    bpy.utils.register_class(AnimusicGeneratorPanel)
    bpy.utils.register_class(AniMusicProperties)
    bpy.utils.register_class(AnimusicGenerate)
    bpy.types.Scene.animusic = PointerProperty(type=AniMusicProperties)


def unregister():
    del bpy.types.Scene.animusic
    bpy.utils.unregister_class(AnimusicGenerate)
    bpy.utils.unregister_class(AniMusicProperties)
    bpy.utils.unregister_class(AnimusicGeneratorPanel)


# This allows you to run the script directly from Blender's Text editor
# to test the add-on without having to install it.
if __name__ == "__main__":
    register()
