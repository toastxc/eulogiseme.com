podman run \
        --name reywen-mongodb -d \
	-p 27017:27017 \
        -e MONGO_INITDB_ROOT_USERNAME="username" \
        -e MONGO_INITDB_ROOT_PASSWORD="password" \
        mongo
