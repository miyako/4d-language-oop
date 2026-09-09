// overload 0
var $receiver : cs.SynthTableEntity
$receiver:=ds.SynthTable.new()
var $entityToCompare1 : cs.SynthTableEntity
$entityToCompare1:=ds.SynthTable.new()
var $result2 : Collection
$result2:=$receiver.diff($entityToCompare1; New collection)
// overload 0 [optional:omit-attributesToCompare]
$receiver:=ds.SynthTable.new()
var $entityToCompare3 : cs.SynthTableEntity
$entityToCompare3:=ds.SynthTable.new()
var $result4 : Collection
$result4:=$receiver.diff($entityToCompare3)
