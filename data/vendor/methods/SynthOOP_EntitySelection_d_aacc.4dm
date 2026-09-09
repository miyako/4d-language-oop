// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : Collection
$result1:=$receiver.distinct("synthText"; dk diacritical)
// overload 0 [enum EntitySelection.distinct.options = dk count values]
$receiver:=ds.SynthTable.all()
var $result2 : Collection
$result2:=$receiver.distinct("synthText"; dk count values)
// overload 0 [optional:omit-options]
$receiver:=ds.SynthTable.all()
var $result3 : Collection
$result3:=$receiver.distinct("synthText")
