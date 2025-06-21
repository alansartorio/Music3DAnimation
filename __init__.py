import bpy

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from bpy.stub_internal.rna_enums import OperatorReturnItems

class AnimusicGenerator(bpy.types.Operator):
    """Keyframe Generator for Animusic"""      # Use this as a tooltip for menu items and buttons.
    bl_idname = "object.generate_animusic"        # Unique identifier for buttons and menu items to reference.
    bl_label = "Generate Animusic Keyframes"         # Display name in the interface.
    bl_options = {'REGISTER', 'UNDO'}  # Enable undo for the operator.

    def execute(self, context) -> set['OperatorReturnItems']:
        scene = context.scene
        assert scene is not None
        for obj in scene.objects:
            obj.location.x += 3.0

        return {'FINISHED'}

def menu_func(self, context):
    self.layout.operator(AnimusicGenerator.bl_idname)

def register():
    bpy.utils.register_class(AnimusicGenerator)
    bpy.types.VIEW3D_MT_object.append(menu_func)

def unregister():
    bpy.utils.unregister_class(AnimusicGenerator)
    bpy.types.VIEW3D_MT_object.remove(menu_func)


# This allows you to run the script directly from Blender's Text editor
# to test the add-on without having to install it.
if __name__ == "__main__":
    register()
