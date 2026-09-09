// overload 0
var $receiver : cs.SynthTableSelection
$receiver:=ds.SynthTable.all()
var $result1 : Collection
$result1:=$receiver.toCollection(dk with primary key; 1; 1)
// overload 0 [enum EntitySelection.toCollection.options = dk with stamp]
$receiver:=ds.SynthTable.all()
var $result2 : Collection
$result2:=$receiver.toCollection(dk with stamp; 1; 1)
// overload 0 [optional:omit-options]
$receiver:=ds.SynthTable.all()
var $result3 : Collection
$result3:=$receiver.toCollection()
// overload 0 [optional:omit-begin]
$receiver:=ds.SynthTable.all()
var $result4 : Collection
$result4:=$receiver.toCollection(dk with primary key)
// overload 0 [optional:omit-howMany]
$receiver:=ds.SynthTable.all()
var $result5 : Collection
$result5:=$receiver.toCollection(dk with primary key; 1)
// overload 1
$receiver:=ds.SynthTable.all()
var $result6 : Collection
$result6:=$receiver.toCollection("synthText"; dk with primary key; 1; 1)
// overload 1 [enum EntitySelection.toCollection.options = dk with stamp]
$receiver:=ds.SynthTable.all()
var $result7 : Collection
$result7:=$receiver.toCollection("synthText"; dk with stamp; 1; 1)
// overload 1 [optional:omit-options]
$receiver:=ds.SynthTable.all()
var $result8 : Collection
$result8:=$receiver.toCollection("synthText")
// overload 1 [optional:omit-begin]
$receiver:=ds.SynthTable.all()
var $result9 : Collection
$result9:=$receiver.toCollection("synthText"; dk with primary key)
// overload 1 [optional:omit-howMany]
$receiver:=ds.SynthTable.all()
var $result10 : Collection
$result10:=$receiver.toCollection("synthText"; dk with primary key; 1)
// overload 2
$receiver:=ds.SynthTable.all()
var $result11 : Collection
$result11:=$receiver.toCollection(New collection; dk with primary key; 1; 1)
// overload 2 [enum EntitySelection.toCollection.options = dk with stamp]
$receiver:=ds.SynthTable.all()
var $result12 : Collection
$result12:=$receiver.toCollection(New collection; dk with stamp; 1; 1)
// overload 2 [optional:omit-options]
$receiver:=ds.SynthTable.all()
var $result13 : Collection
$result13:=$receiver.toCollection(New collection)
// overload 2 [optional:omit-begin]
$receiver:=ds.SynthTable.all()
var $result14 : Collection
$result14:=$receiver.toCollection(New collection; dk with primary key)
// overload 2 [optional:omit-howMany]
$receiver:=ds.SynthTable.all()
var $result15 : Collection
$result15:=$receiver.toCollection(New collection; dk with primary key; 1)
