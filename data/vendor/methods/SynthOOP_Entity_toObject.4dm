// overload 0
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $result1 : Object
$result1:=$receiver.toObject()
// overload 1
$receiver:=ds.SynthTable.new()
var $result2 : Object
$result2:=$receiver.toObject("synthText"; dk with primary key)
// overload 1 [enum Entity.toObject.options = dk with stamp]
$receiver:=ds.SynthTable.new()
var $result3 : Object
$result3:=$receiver.toObject("synthText"; dk with stamp)
// overload 1 [optional:omit-options]
$receiver:=ds.SynthTable.new()
var $result4 : Object
$result4:=$receiver.toObject("synthText")
// overload 2
$receiver:=ds.SynthTable.new()
var $result5 : Object
$result5:=$receiver.toObject(New collection; dk with primary key)
// overload 2 [enum Entity.toObject.options = dk with stamp]
$receiver:=ds.SynthTable.new()
var $result6 : Object
$result6:=$receiver.toObject(New collection; dk with stamp)
// overload 2 [optional:omit-options]
$receiver:=ds.SynthTable.new()
var $result7 : Object
$result7:=$receiver.toObject(New collection)
