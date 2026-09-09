// overload 0
var $receiver : 4D.DataClass
$receiver:=ds.SynthTable
var $result1 : cs.SynthTableSelection
$result1:=$receiver.all(New object)
// overload 0 [optional:omit-settings]
$receiver:=ds.SynthTable
var $result2 : cs.SynthTableSelection
$result2:=$receiver.all()
