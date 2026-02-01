<?php
header("Cache-Control: no-cache");
header("Content-Type: text/html");

$protocol = $_SERVER['SERVER_PROTOCOL'] ?? '';
$method = $_SERVER['REQUEST_METHOD'] ?? '';
$query_string = $_SERVER['QUERY_STRING'] ?? '';
$form_data = file_get_contents('php://input');

print "<!DOCTYPE html>";
print "<html><head><title>General Request Echo</title>";
print "</head><body><h1 align='center'>General Request Echo</h1>";
print "<hr>";
print "<p><b>HTTP Protocol:</b> $protocol</p>";
print "<p><b>HTTP Method:</b> $method</p>";
print "<p><b>Query String:</b> $query_string</p>";
print "<p><b>Message Body:</b> $form_data</p>";
print "</body></html>";
?>
