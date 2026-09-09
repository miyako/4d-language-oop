// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $entity1 : cs.SynthTableEntity
$entity1:=ds.SynthTable.new()
var $result2 : cs.SynthTableSelection
$result2:=$receiver.minus($entity1; dk keep ordered)
// overload 0 [optional:omit-keepOrder]
$receiver:=ds.SynthTable.all()
var $entity3 : cs.SynthTableEntity
$entity3:=ds.SynthTable.new()
var $result4 : cs.SynthTableSelection
$result4:=$receiver.minus($entity3)
// overload 1
$receiver:=ds.SynthTable.all()
var $entitySelection5 : cs.SynthTableSelection
$entitySelection5:=ds.SynthTable.all()
var $result6 : cs.SynthTableSelection
$result6:=$receiver.minus($entitySelection5; dk keep ordered)
// overload 1 [optional:omit-keepOrder]
$receiver:=ds.SynthTable.all()
var $entitySelection7 : cs.SynthTableSelection
$entitySelection7:=ds.SynthTable.all()
var $result8 : cs.SynthTableSelection
$result8:=$receiver.minus($entitySelection7)
