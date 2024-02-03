# About this

This application can manage TODO on web browser, written by Rust.
You can run this app on Google Cloud Platform resources, like Cloud Run & GKE.  
(Test environment: Cloud Run + Cloud SQL) 

# Usage

## 1. Installation

Pull this app's image from Docker Hub:
```shell
docker pull htatsuhiro/todo_webapp_rs:latest
```

## 2. Setting MySQL Database

### Use Google Cloud SQL




### Use your Database server
You should migrate on your DB server with `migrations/up.sql`.  
For example (use sqlx-cli):
```shell
sqlx migrate run 
```

## 3. Run App
Run this image:
```shell
docker run -d --name APP -p PORT:80 htatsuhiro/todo_webapp_rs:latest
```
`APP` is container name.  
`PORT` is host machine's port, can listen this app. 

## 3. Stop & Delete App
Stop running container:
```shell
docker stop APP
```
You can delete this app's image from host machine:
```shell
docker image rm htatsuhiro/todo_webapp_rs:latest
```