#!/usr/bin/env bash

case $1 in
    '{"action":"snapshot-list"}')
        echo $1 1>&2
        echo '{"ffd1a106-29c7-4f8e-821e-230d4ca1082d":{"comment":"test2","state":"completed","timestamp":1484740149842269, "size": 42},"ffeb1040-b3ea-479e-9fa8-ea6c3408e880":{"comment":"test","state":"completed","timestamp":1484740141975811}}'
        ;;
    '{"action":"backup-list"}')
        echo '{"ff7c4d16-2b8f-41a2-8faa-3ae301b27f81":{"comment":"test","files":{"7032de1c-e5c5-e7f5-8eb6-b7ee1eff2fde\/ff7c4d16-2b8f-41a2-8faa-3ae301b27f81":{"size":41943040}},"pending":true,"state":"uploading","timestamp":1484745596506474}}'
        ;;
    '{"action":"cluster-vms"}')
        echo '["ff7c4d16-2b8f-41a2-8faa-3ae301b27f81"]'
        ;;
    *)
        echo $1
    ;;
esac
