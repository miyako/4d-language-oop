// property read
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read1 : Text
$read1:=$receiver.certificateName
// property write
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
$receiver.certificateName:="synthText"
