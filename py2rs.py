import sys
import ast
import json

# static TYPEINFOS: &'static [TypeInfo] = &[
#     TypeInfo::Int { min: 0, bits: 7 },
#     TypeInfo::Struct {
#         items: &[
#             ("aaa", 0, 0),
#         ]
#     }
# ];

class ProtocolDefinition(object):
    def __init__(self, module):
        assigns = dict()
        for element in module.body:
            if isinstance(element, ast.Assign) and len(element.targets) == 1:
                name = element.targets[0].id
                assigns[name] = ast.literal_eval(element.value)
        self.typeinfos = assigns['typeinfos']
        self.game_event_types = assigns['game_event_types']


def load_module(filename):
    with open(filename) as fh:
        return ast.parse(fh.read())

def load_protocol(filename):
    return ProtocolDefinition(load_module(filename))


def typeinfo_array_to_rs(bounds, typeid):
    yield "TypeInfo::Array {{ bounds: ({}, {}), typeid: {} }},".\
        format(bounds[0], bounds[1], typeid)


def typeinfo_bitarray_to_rs(bounds):
    yield "TypeInfo::BitArray {{ len_min: {}, len_bits: {} }},".\
        format(bounds[0], bounds[1])

def typeinfo_blob_to_rs(bounds):
    yield "TypeInfo::Blob {{ len_min: {}, len_bits: {} }},".\
        format(bounds[0], bounds[1])


def typeinfo_bool_to_rs():
    yield "TypeInfo::Bool,"


def typeinfo_choice_to_rs(bounds, fields):
    yield "TypeInfo::Choice {"
    yield "    min: {},".format(bounds[0])
    yield "    bits: {},".format(bounds[1])
    yield "    types: &["
    for field in fields:
        pass
    yield "    ]"
    yield "},"


def typeinfo_fourcc_to_rs():
    yield "TypeInfo::FourCC,"


def typeinfo_int_to_rs(bounds):
    yield "TypeInfo::Int {{ min: {}, bits: {} }},".\
        format(bounds[0], bounds[1])


def typeinfo_null_to_rs():
    yield "TypeInfo::Null,"


def typeinfo_optional_to_rs(typeid):
    yield "TypeInfo::Optional {{ typeid: {} }},".\
        format(typeid)


def typeinfo_real32_to_rs(args):
    yield "TypeInfo::Real32,"


def typeinfo_real64_to_rs(args):
    yield "TypeInfo::Real64,"


def typeinfo_struct_to_rs(fields):
    yield "TypeInfo::Struct {"
    yield "    items: &["
    for (name, typeid, tag) in fields:
        yield "        ({}, {}, {}),".format(json.dumps(name), typeid, tag)
    yield "    ],"
    yield "},"


typeinfo_to_rs_map = {
    '_array': typeinfo_array_to_rs,
    '_bitarray': typeinfo_bitarray_to_rs,
    '_blob': typeinfo_blob_to_rs,
    '_bool': typeinfo_bool_to_rs,
    '_choice': typeinfo_choice_to_rs,
    '_fourcc': typeinfo_fourcc_to_rs,
    '_int': typeinfo_int_to_rs,
    '_null': typeinfo_null_to_rs,
    '_optional': typeinfo_optional_to_rs,
    '_real32': typeinfo_real32_to_rs,
    '_real64': typeinfo_real64_to_rs,
    '_struct': typeinfo_struct_to_rs,
}

def typeinfo_to_rs(typeinfo):
    return typeinfo_to_rs_map[typeinfo[0]](*typeinfo[1])

if __name__ == '__main__':
    protocol = load_protocol(sys.argv[1])
    print('''static TYPEINFOS: &'static [TypeInfo] = &[''')
    for typeinfo in protocol.typeinfos:
        for line in typeinfo_to_rs(typeinfo):
            print("    {}".format(line))
    print('''];''')
    print('''''')