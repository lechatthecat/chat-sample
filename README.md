# chat-sample

# Preparation
Please install the following beforehand
- minikube
- skaffold
- git

# minikube start
$ git clone https://github.com/lechatthecat/chat-sample
$ cd chat-sample
$ minikube start --mount --mount-string="$HOME/Documents/chat-sample/code:/code" --driver=docker
If you start this environemnt for the first time, please enable ingress
$ minikube addons enable ingress

Deploy your pods:
$ skaffold delete & skaffold run --force

# Create tables and dumm data
Please install the following
$ cargo install sqlx-cli

If you want to access the database etc:
$ minikube kubectl port-forward svc/postgres 5432:5432
$ minikube kubectl port-forward svc/mongodb 27017:27017
$ minikube kubectl port-forward svc/redis-cluster 6379:6379

(please see .env for the database connection information)

Please make tables by: init.sql
Please make dummy data by: dummy_data.sql

# Stopping minikube
$ minikube stop
If you don't want minikube's envrionment anymore:
$ minikube delete
