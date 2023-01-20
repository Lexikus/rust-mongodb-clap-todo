mongo:
	docker network create mongo-network
	docker run --network mongo-network -p 27017:27017 -d --name mongodb bitnami/mongodb:latest
	docker run --network mongo-network -e ME_CONFIG_MONGODB_SERVER=mongodb -p 8081:8081 -d --name mongo-express mongo-express