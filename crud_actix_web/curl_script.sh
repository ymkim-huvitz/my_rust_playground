#!/bin/bash 


#curl -X GET -H "Content-Type: application/json" http://localhost:8081/api/todos

#curl -X GET -H "Content-Type: application/json" http://localhost:8081/api/todos | jq

#curl -X POST -H "Content-Type: application/json" -d '{ "title": "from curl2 ", "content": "hello curl 2" }' http://localhost:8081/api/todos


# Https
#curl -X GET -H "Content-Type: application/json" https://localhost:18081/api/todos | jq


#Health Check
#curl http://localhost:8081/api/healthchecker

#Get All
#curl -X GET -H "Content-Type: application/json" http://localhost:8081/api/todos | jq

#Create
#curl -X POST -H "Content-Type: application/json" -d '{ "title": "from curl", "content": "hello curl" }' http://localhost:8081/api/todos

#Modify
#$ curl -X PATCH -H "Content-Type: application/json" -d '{ "content": "curl update", "completed": true }' http://localhost:8081/api/todos/41b5c843-6cce-4bbe-abed-1a67de5683f6

#Delete
#$ curl -X DELETE http://localhost:8081/api/todos/41b5c843-6cce-4bbe-abed-1a67de5683f6

