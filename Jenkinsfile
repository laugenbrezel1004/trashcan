pipeline {
    agent {
        dockerContainer {
            image 'rust:latest'
            //args '-u root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
    }
    post {
        always {
            cleanWs()
        }
    }
}

