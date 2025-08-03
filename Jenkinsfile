pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '--user root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }

    environment {
        GITHUB_TOKEN = credentials('github-pat')
        GITHUB_REPO = 'laugenbrezel1004/trashcan'
        // RELEASE_TAG wird dynamisch generiert, daher entfernen wir die statische Definition
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Test') {
            steps {
                sh 'rustup component add clippy'
                sh 'cargo clippy --all-targets --all-features'
                sh 'cargo test'
            }
        }

        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }

        stage('Archive') {
            steps {
                archiveArtifacts artifacts: 'target/release/trashcan', allowEmptyArchive: true
            }
        }

        stage('Create GitHub Release') {
            steps {
                script {

                    // Erstelle den GitHub-Release
                    createGitHubRelease(
                        credentialId: 'github-pat',
                        repository: "${GITHUB_REPO}",
                        tag: "${newTag}",
                        title: "Release ${newTag}",
                        releaseNotes: "Automatisch generierte Release ${newTag}",
                        commitish: 'main' // Stelle sicher, dass dies dein Standard-Branch ist
                    )

                    // Lade das Asset hoch
                    uploadGithubReleaseAsset(
                        credentialId: 'github-pat',
                        repository: "${GITHUB_REPO}",
                        tagName: "${newTag}",
                        uploadAssets: [
                            [filePath: 'target/release/*']
                        ]
                    )
                }
            }
        }
    }

    post {
        failure {
            echo 'Test or Linting failed, aborting Pipeline'
        }
    }
}