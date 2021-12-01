import jsony, json
import tables
import strutils
import std/re

include mathparse

type 
    DataType = enum
        Int,
        Float,
        String,
        Seq,
        SeqTable,
        Hash
    Data = object
        case `type`: DataType
        of Int: intVal: int
        of Float: fltVal: float
        of String: strVal: string
        of Seq: seqVal: seq[Data]
        of SeqTable: seqTableVal: seq[Table[string, Data]]
        of Hash: hashVal: Table[string, Data]

template unpackIt(d: Data, body: untyped): untyped =
    case d.`type`
    of Int:
        var it {.inject.} = d.intVal
        body
    of Float:
        var it {.inject.} = d.fltVal
        body
    of String:
        var it {.inject.} = d.strVal
        body
    of Seq:
        var it {.inject.} = d.seqVal
        body
    of SeqTable:
        var it {.inject.} = d.seqTableVal
        body
    of Hash:
        var it {.inject.} = d.hashVal
        body
    

proc newData[T: int or float or string or seq[Data] or seq[Table[string, Data]] or Table[string, Data]](value: T): Data =
    when T is int:
        return Data(`type`: Int, intVal: value)
    elif T is float:
        return Data(`type`: Float, fltVal: value)
    elif T is string:
        return Data(`type`: String, strVal: value)
    elif T is seq[Data]:
        return Data(`type`: Seq, seqVal: value)
    elif T is seq[Table[string, Data]]:
        return Data(`type`: SeqTable, seqTableVal: value)
    elif T is Table[string, Data]:
        return Data(`type`: Hash, hashVal: value)

proc toData[T: JsonNode or seq[Data] or seq[Table[string, Data]] or Table[string, Data]](value: T, dataType: string): Data =
    when T is JsonNode:
        if dataType == "int":
            return newData(to(value,int))
        elif dataType == "float":
            return newData(to(value,float))
        elif dataType == "string":
            return newData($value)
    else:
        return newData(value)

type Variable = object
    name: string
    data_type: string
    required: bool

type ModelDict = object
    modelList: seq[string]
    model: Table[string, seq[Variable]]

# let pokedex = fromJson(readFile("../data/pokedex.json"))
let modelsDict = fromJson(readFile("../data/models.json"), ModelDict)

proc parseModel(model: seq[Variable], data: JsonNode): Table[string, Data] =
    for variable in model:
        if variable.data_type =~ re"model\(([^)]+)\)":
            var table = parseModel(model = modelsDict.model[matches[0]], data = data[variable.name])
            for k,v in table:
                result[variable.name & "." & k] = v
        elif variable.data_type =~ re"seq\[([^]]+)\]":
            if matches[0] =~ re"model\(([^)]+)\)":
                var dataSeq: seq[Table[string, Data]]
                for val in data[variable.name]:
                    dataSeq.add(parseModel(model = modelsDict.model[matches[0]], data = val))
                result[variable.name] = toData(dataSeq, "seq")
            else:
                var dataSeq: seq[Data]
                for val in data[variable.name]:
                    dataSeq.add(toData(val, matches[0]))
                result[variable.name] = toData(dataSeq, "seq")
                
        elif variable.data_type =~ re"hash\[([^]]+)\]":
            if matches[0] =~ re"model\(([^)]+)\)":
                for key,val in data[variable.name]:
                    result[variable.name & "." & key] = toData(parseModel(model = modelsDict.model[matches[0]], data = val), "hash")
            else:
                for key,val in data[variable.name]:
                    result[variable.name & "." & key] = toData(val, matches[0])
                
        else:
            result[variable.name] = toData(data[variable.name], variable.data_type)
