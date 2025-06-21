import mathutils

def note_to_position(track: int, note: int) -> mathutils.Vector:
    return mathutils.Vector((note, 0, 0))
