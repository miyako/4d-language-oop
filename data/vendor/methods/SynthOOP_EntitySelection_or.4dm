// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $entity1 : cs.SynthTableEntity
$entity1:=ds.SynthTable.new()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.or($entity1)
// overload 1
$receiver:=ds.SynthTable.all()
var $entitySelection3 : cs.SynthTableSelection
$entitySelection3:=ds.SynthTable.all()
var $result4 : cs.SynthTableSelection
$result4:=$receiver.or($entitySelection3)
