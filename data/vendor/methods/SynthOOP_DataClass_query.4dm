// overload 0
var $receiver : 4D.DataClass
$receiver:=ds.SynthTable
var $result1 : cs.SynthTableSelection
$result1:=$receiver.query("synthText"; "synthText"; New object)
// overload 0 [optional:omit-querySettings]
$receiver:=ds.SynthTable
var $result2 : cs.SynthTableSelection
$result2:=$receiver.query("synthText"; "synthText")
// overload 1
$receiver:=ds.SynthTable
var $result3 : cs.SynthTableSelection
$result3:=$receiver.query(New object; New object)
// overload 1 [optional:omit-querySettings]
$receiver:=ds.SynthTable
var $result4 : cs.SynthTableSelection
$result4:=$receiver.query(New object)
