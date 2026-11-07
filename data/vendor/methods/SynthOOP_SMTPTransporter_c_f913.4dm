// property read [4D.File]
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read1 : 4D.File
$read1:=$receiver.certificate
// property write [4D.File]
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $certificate2 : 4D.File
$certificate2:=File("/PACKAGE/README.md")
$receiver.certificate:=$certificate2
// property read [Blob]
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read3 : Blob
$read3:=$receiver.certificate
// property write [Blob]
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $certificate4 : Blob
$receiver.certificate:=$certificate4
