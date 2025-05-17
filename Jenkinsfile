pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '--user root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }

    environment {
        GITHUB_TOKEN = credentials('github-pat') // Jenkins Credential ID for the PAT
        GITHUB_REPO = 'laugenbrezel1004/trashcan' // Replace with your repo (e.g., 'meinname/meinprojekt')
        RELEASE_TAG = "v1.0.${env.BUILD_NUMBER}" // Dynamic tag, e.g., v1, v2, ...
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        // stage('test') {
        //   steps {
        //     sh 'cargo test'
        //   }
        // }

        stage('build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Archive') {
            steps {
                archiveArtifacts artifacts: 'target/release/*', allowEmptyArchive: true
            }
        }

        stage('Create GitHub Release') {
            steps {
                createGitHubRelease(
                    credentialId: 'github-pat', // ID of the GitHub Personal Access Token
                    repository: 'laugenbrezel1004/trashcan', // Your repository
                    tag: "${RELEASE_TAG}", // Tag for the release
                    commitish: 'main', // Branch or commit SHA
                    assets: 'target/release/trashcan'
                    prerelease: false // Pre-release status
                )
            }
        }
    }

    post {
        always {
            cleanWs()
        }
    }
}
