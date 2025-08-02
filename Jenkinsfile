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
                    // Hole die aktuelle Version aus Cargo.toml
                    def newVersion = sh(script: "awk -F'\"' '/^version\s*=\s*\"/ {gsub(/\./, \"\", $2); print $2 + 1}' Cargo.toml", returnStdout: true).trim()                    echo "Cargo.toml Version: ${cargoVersion}"
                    echo newVersion


                    // Falls keine Releases existieren, starte mit der Version aus Cargo.toml
                    if (!latestTag || latestTag == 'null') {
                        latestTag = "v${cargoVersion}"
                    }

                    // Parse die aktuelle Versionsnummer (entferne 'v' falls vorhanden)
                    def version = latestTag.startsWith('v') ? latestTag.replace('v', '') : latestTag
                    def versionParts = version.split('\\.')
                    def major = versionParts[0].toInteger()
                    def minor = versionParts[1].toInteger()
                    def patch = versionParts[2].toInteger()

                    // Erhöhe die Patch-Version
                    def newPatch = patch + 1
                    def newVersion = "${major}.${minor}.${newPatch}"
                    def newTag = "v${newVersion}"

                    // Optional: Aktualisiere Cargo.toml mit der neuen Version
                    sh """
                        sed -i 's/version = "${cargoVersion}"/version = "${newVersion}"/' Cargo.toml
                        git add Cargo.toml
                        git commit -m "Bump version to ${newVersion}"
                        git push
                    """

                    echo "Neue Versionsnummer: ${newTag}"

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