import bpy
from pathlib import Path

from typing import TYPE_CHECKING, assert_never

from bpy.types import Panel, PropertyGroup
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
    # bl_context = "object"
    # bl_options = {'DEFAULT_CLOSED'}

    @classmethod
    def poll(cls, context):
        return context.object is not None

    def draw_header(self, context):
        layout = self.layout
        assert layout is not None
        layout.label(text="Animusic")

    def run(self, context):
        scene = context.scene
        assert scene is not None

        obj = scene.objects["Target"]
        obj.location = demo1.note_to_position(0, 10)
        obj.keyframe_insert(data_path="location", frame=1)

    def draw(self, context):
        layout = self.layout
        assert layout is not None
        scene = context.scene
        assert scene is not None

        col = layout.column(align=True)
        col.prop(scene.animusic, "director_data", text="")
        col.operator("object.generate_animusic_keyframes", text="Import")


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
        )  # make a path object of abs path

        assert director_data_path.exists()

        with open(director_data_path) as director_data_file:
            director_data = DirectorData.from_json(json.load(director_data_file))

        print(director_data)

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
