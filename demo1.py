import mathutils

def note_to_position(track: int, note: int, preparing: bool) -> mathutils.Vector:
    return mathutils.Vector((note, 0, 5 if preparing else 0))

prepare_time = 0.1
release_time = 0.2
