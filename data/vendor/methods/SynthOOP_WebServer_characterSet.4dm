// property read [Number]
var $receiver : 4D.WebServer
$receiver:=WEB Server
var $read1 : Real
$read1:=$receiver.characterSet
// property write [Number]
$receiver:=WEB Server
$receiver.characterSet:=1
// property read [Text]
$receiver:=WEB Server
var $read2 : Text
$read2:=$receiver.characterSet
// property write [Text]
$receiver:=WEB Server
$receiver.characterSet:="synthText"
