// overload 0
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $result1 : Object
$result1:=$receiver.lock(dk reload if stamp changed)
// overload 0 [optional:omit-mode]
$receiver:=ds.SynthTable.new()
var $result2 : Object
$result2:=$receiver.lock()
