// overload 0
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $result1 : Variant
$result1:=$receiver.getKey(dk key as string)
// overload 0 [optional:omit-mode]
$receiver:=ds.SynthTable.new()
var $result2 : Variant
$result2:=$receiver.getKey()
