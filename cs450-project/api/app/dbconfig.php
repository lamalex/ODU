<?php 

namespace Appconfig;

function load_db_config(
    $defaultHost = '',
    $defaultUser = '',
    $defaultPass = '',
    $defaultDb = '')
{
    $cleardb_url = getenv("CLEARDB_DATABASE_URL");
    $cleardb_conn_params = array_filter(parse_url($cleardb_url));

    return array(
            "host" => $cleardb_conn_params["host"] ?? $defaultHost,
            "user" => $cleardb_conn_params["user"] ?? $defaultUser,
            "pass" => $cleardb_conn_params["pass"] ?? $defaultPass,
            "name" => $cleardb_url
                ? substr($cleardb_conn_params["path"], 1) 
                : $defaultDb,
    );
}
