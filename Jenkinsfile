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
                     def version = sh(script: "grep '^version' Cargo.toml | cut -d '\"' -f 2", returnStdout: true).trim()
                     echo "${version}"

                    // Erstelle den GitHub-Release
                    createGitHubRelease(
                        credentialId: 'github-pat',
                        repository: "${GITHUB_REPO}",
                        tag: version,
                        title: "Release ${version}",
                        releaseNotes: "Automatisch generierte Release $version}",
                        commitish: 'main' // Stelle sicher, dass dies dein Standard-Branch ist
                    )

                    // Lade das Asset hoch
                    uploadGithubReleaseAsset(
                        credentialId: 'github-pat',
                        repository: "${GITHUB_REPO}",
                        //tagName: "v${version}",
                        uploadAssets: [
                            [filePath: 'target/release/trashcan']
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