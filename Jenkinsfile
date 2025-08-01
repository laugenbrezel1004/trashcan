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
        RELEASE_TAG = "v1.0.${env.BUILD_NUMBER}"
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
                //sh 'cargo clippy --all-targets --all-features -- -D warnings'
                // d warnings exits if any kind of warning is found
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
                archiveArtifacts artifacts: 'target/release/*', allowEmptyArchive: true
            }
        }


         stage('Create GitHub Release') {
             steps {
                 script {
                     // Extrahiere die Version aus Cargo.toml
                     def version = sh(script: "grep '^version' Cargo.toml | cut -d '\"' -f 2", returnStdout: true).trim()
                     echo "Version: ${version}"
                     // Erstelle den GitHub-Release
                     createGitHubRelease(
                         credentialId: 'github-pat', // ID des GitHub Personal Access Tokens
                         repository: 'laugenbrezel1004/trashcan', // Dein Repository
                         tag: "v${version}", // Tag für den Release (z. B. v0.1.0)
                         commitish: 'main', // Branch oder Commit-SHA
                         //assets: 'target/release/trashcan' // Optional, falls direkt hier hochgeladen
                     )
                     // Lade das Asset hoch
                     uploadGithubReleaseAsset(
                         credentialId: 'github-pat',
                         repository: 'laugenbrezel1004/trashcan',
                         tagName: "v${version}",
                         uploadAssets: [
                             [filePath: 'trashcan']
                         ]
                     )
                 }
             }
         }

    } //stages

    post {
        failure {
            echo 'Test or Linting failed, aborting Pipeline'
        }
    }
}