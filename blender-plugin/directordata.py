from dataclasses import dataclass
from typing import SupportsFloat


@dataclass
class Note:
    delta: float
    track: int
    note: int

    @classmethod
    def from_json(cls, json_data):
        assert isinstance(json_data, dict)

        assert "delta" in json_data
        delta = json_data["delta"]
        assert isinstance(delta, SupportsFloat)
        delta = float(delta)

        assert "track" in json_data
        track = json_data["track"]
        assert isinstance(track, int)

        assert "note" in json_data
        note = json_data["note"]
        assert isinstance(note, int)

        return cls(delta, track, note)


@dataclass
class Actuator:
    name: str
    notes: list[Note]

    @classmethod
    def from_json(cls, json_data):
        assert isinstance(json_data, dict)

        assert "name" in json_data
        name = json_data["name"]
        assert isinstance(name, str)

        assert "notes" in json_data
        notes = json_data["notes"]
        assert isinstance(notes, list)

        return cls(name, list(map(Note.from_json, notes)))


@dataclass
class DirectorData:
    actuators: list[Actuator]

    @classmethod
    def from_json(cls, json_data):
        assert isinstance(json_data, dict)

        assert "actuators" in json_data
        actuators = json_data["actuators"]
        assert isinstance(actuators, list)

        return cls(list(map(Actuator.from_json, actuators)))
