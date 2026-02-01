<?php
header("Cache-Control: no-cache");
header("Content-Type: text/html");

$method = $_SERVER['REQUEST_METHOD'];
$date = date('Y-m-d H:i:s');
$ip = $_SERVER['REMOTE_ADDR'];
$user_agent = $_SERVER['HTTP_USER_AGENT'];
$hostname = gethostname();

if ($method == 'GET') {
    $data = $_GET;
} elseif ($method == 'POST') {
    $content_type = isset($_SERVER['CONTENT_TYPE']) ? $_SERVER['CONTENT_TYPE'] : '';
    if (strpos($content_type, 'application/json') !== false) {
        $json = file_get_contents('php://input');
        $data = json_decode($json, true);
    } else {
        $data = $_POST;
    }
} elseif ($method == 'PUT' || $method == 'DELETE') {
    $content_type = isset($_SERVER['CONTENT_TYPE']) ? $_SERVER['CONTENT_TYPE'] : '';
    $input = file_get_contents('php://input');
    if (strpos($content_type, 'application/json') !== false) {
        $data = json_decode($input, true);
    } else {
        parse_str($input, $data);
    }
} else {
    $data = array();
}

print "<!DOCTYPE html>";
print "<html>";
print "<head><title>PHP Echo</title></head>";
print "<body>";
print "<h1>PHP Echo</h1>";
print "<p><strong>Method:</strong> $method</p>";
print "<p><strong>Hostname:</strong> $hostname</p>";
print "<p><strong>Date/Time:</strong> $date</p>";
print "<p><strong>User Agent:</strong> $user_agent</p>";
print "<p><strong>IP:</strong> $ip</p>";

print "<h2>Data Received</h2>";
if (!empty($data)) {
    print "<table border='1'>";
    print "<tr><th>Key</th><th>Value</th></tr>";
    foreach ($data as $key => $value) {
        if ($key != 'language' && $key != 'method' && $key != 'encoding') {
            print "<tr><td>$key</td><td>$value</td></tr>";
        }
    }
    print "</table>";
} else {
    print "<p>No data</p>";
}

print "</body></html>";
?>