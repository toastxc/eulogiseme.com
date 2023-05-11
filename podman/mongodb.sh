podman run \
        --name libby-mongodb -d \
        -e MONGO_INITDB_ROOT_USERNAME="username" \
        -e MONGO_INITDB_ROOT_PASSWORD="password" \
        --pod 'eulogise.com' \
        mongo