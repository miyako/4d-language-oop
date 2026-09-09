// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : cs.SynthTableSelection
$result1:=$receiver.orderByFormula("synthText"; dk ascending; New object)
// overload 0 [enum EntitySelection.orderByFormula.sortOrder = dk descending]
$receiver:=ds.SynthTable.all()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.orderByFormula("synthText"; dk descending; New object)
// overload 0 [optional:omit-sortOrder]
$receiver:=ds.SynthTable.all()
var $result3 : cs.SynthTableSelection
$result3:=$receiver.orderByFormula("synthText")
// overload 0 [optional:omit-settings]
$receiver:=ds.SynthTable.all()
var $result4 : cs.SynthTableSelection
$result4:=$receiver.orderByFormula("synthText"; dk ascending)
// overload 1
$receiver:=ds.SynthTable.all()
var $result5 : cs.SynthTableSelection
$result5:=$receiver.orderByFormula(New object; dk ascending; New object)
// overload 1 [enum EntitySelection.orderByFormula.sortOrder = dk descending]
$receiver:=ds.SynthTable.all()
var $result6 : cs.SynthTableSelection
$result6:=$receiver.orderByFormula(New object; dk descending; New object)
// overload 1 [optional:omit-sortOrder]
$receiver:=ds.SynthTable.all()
var $result7 : cs.SynthTableSelection
$result7:=$receiver.orderByFormula(New object)
// overload 1 [optional:omit-settings]
$receiver:=ds.SynthTable.all()
var $result8 : cs.SynthTableSelection
$result8:=$receiver.orderByFormula(New object; dk ascending)
