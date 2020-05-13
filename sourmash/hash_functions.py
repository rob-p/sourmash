from enum import Enum

from ._lowlevel import lib


class HashFunctions(Enum):
    murmur64_DNA = lib.HASH_FUNCTIONS_MURMUR64_DNA
    murmur64_protein = lib.HASH_FUNCTIONS_MURMUR64_PROTEIN
    murmur64_dayhoff = lib.HASH_FUNCTIONS_MURMUR64_DAYHOFF
    murmur64_hp = lib.HASH_FUNCTIONS_MURMUR64_HP

    @classmethod
    def from_string(cls, hash_str):
        hash_str = hash_str.lower()

        if hash_str in ("0.murmur64_dna", "dna"):
            return cls.murmur64_DNA
        elif hash_str in ("0.murmur64_protein", "protein"):
            return cls.murmur64_protein
        elif hash_str in ("0.murmur64_dayhoff", "dayhoff"):
            return cls.murmur64_dayhoff
        elif hash_str in ("0.murmur64_hp", "hp"):
            return cls.murmur64_hp
        else:
            raise Exception("unknown molecule type: {}".format(hash_str))
