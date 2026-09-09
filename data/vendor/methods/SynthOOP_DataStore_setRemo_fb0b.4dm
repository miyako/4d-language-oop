// overload 0
var $receiver : 4D.DataStore
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; "synthText"; "synthText"; 1)
// overload 0 [optional:omit-contextType]
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; "synthText")
// overload 0 [optional:omit-pageLength]
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; "synthText"; "synthText")
// overload 1
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; New collection; "synthText"; 1)
// overload 1 [optional:omit-contextType]
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; New collection)
// overload 1 [optional:omit-pageLength]
$receiver:=ds
$receiver.setRemoteContextInfo("synthText"; "synthText"; New collection; "synthText")
// overload 2
$receiver:=ds
var $dataClassObject1 : 4D.DataClass
$dataClassObject1:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject1; "synthText"; "synthText"; 1)
// overload 2 [optional:omit-contextType]
$receiver:=ds
var $dataClassObject2 : 4D.DataClass
$dataClassObject2:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject2; "synthText")
// overload 2 [optional:omit-pageLength]
$receiver:=ds
var $dataClassObject3 : 4D.DataClass
$dataClassObject3:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject3; "synthText"; "synthText")
// overload 3
$receiver:=ds
var $dataClassObject4 : 4D.DataClass
$dataClassObject4:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject4; New collection; "synthText"; 1)
// overload 3 [optional:omit-contextType]
$receiver:=ds
var $dataClassObject5 : 4D.DataClass
$dataClassObject5:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject5; New collection)
// overload 3 [optional:omit-pageLength]
$receiver:=ds
var $dataClassObject6 : 4D.DataClass
$dataClassObject6:=ds.SynthTable
$receiver.setRemoteContextInfo("synthText"; $dataClassObject6; New collection; "synthText")
