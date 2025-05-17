pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '--user root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
         stage('Test') {
                     steps {
                         sh 'cargo test'
                     }
                 }
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

    }
    post {
        always {
            cleanWs()
        }
    }
}

