// overload 0
var $receiver : 4D.DataClass
$receiver:=ds.SynthTable
var $result1 : cs.SynthTableEntity
$result1:=$receiver.get(1; New object)
// overload 0 [optional:omit-settings]
$receiver:=ds.SynthTable
var $result2 : cs.SynthTableEntity
$result2:=$receiver.get(1)
// overload 1
$receiver:=ds.SynthTable
var $result3 : cs.SynthTableEntity
$result3:=$receiver.get("synthText"; New object)
// overload 1 [optional:omit-settings]
$receiver:=ds.SynthTable
var $result4 : cs.SynthTableEntity
$result4:=$receiver.get("synthText")
