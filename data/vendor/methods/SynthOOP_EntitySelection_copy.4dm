// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : cs.SynthTableSelection
$result1:=$receiver.copy(ck shared)
// overload 0 [optional:omit-option]
$receiver:=ds.SynthTable.all()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.copy()
