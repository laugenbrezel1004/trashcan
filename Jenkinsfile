pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '-u root'
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

