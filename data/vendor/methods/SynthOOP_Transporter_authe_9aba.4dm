// property read
// Transporter is abstract; synthesized against its concrete subclass SMTPTransporter.
var $receiver : 4D.SMTPTransporter
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
var $read1 : Text
$read1:=$receiver.authenticationMode
// property write
// Transporter is abstract; synthesized against its concrete subclass SMTPTransporter.
$receiver:=4D.SMTPTransporter.new(New object("host"; "smtp.example.com"))
$receiver.authenticationMode:="synthText"
