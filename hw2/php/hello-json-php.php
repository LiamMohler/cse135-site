<?php
print "Cache-Control: no-cache\n";
print "Content-Type: application/json\n\n";

$date = date('Y-m-d H:i:s');
$ip = $_SERVER['REMOTE_ADDR'];

$data = array(
    'title' => 'Hello, PHP!',
    'heading' => 'Hello, PHP!',
    'message' => 'This page was generated with the PHP programming language',
    'team' => 'Liam',
    'time' => $date,
    'ip' => $ip
);

print json_encode($data);
?>