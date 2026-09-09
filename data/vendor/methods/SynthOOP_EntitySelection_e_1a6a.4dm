// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : Collection
$result1:=$receiver.extract("synthText"; ck keep null)
// overload 0 [optional:omit-option]
$receiver:=ds.SynthTable.all()
var $result2 : Collection
$result2:=$receiver.extract("synthText")
// overload 1
$receiver:=ds.SynthTable.all()
var $result3 : Collection
$result3:=$receiver.extract("synthText"; "synthText")
