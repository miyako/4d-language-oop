// overload 0
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $entitySelection1 : cs.SynthTableSelection
$entitySelection1:=ds.SynthTable.all()
var $result2 : Integer
$result2:=$receiver.indexOf($entitySelection1)
// overload 0 [optional:omit-entitySelection]
$receiver:=ds.SynthTable.new()
var $result3 : Integer
$result3:=$receiver.indexOf()
