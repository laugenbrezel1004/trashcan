pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '--user root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    environment {
            GITHUB_TOKEN = credentials('github-pat') // Jenkins Credential ID für den PAT
            GITHUB_REPO = 'laugenbrezel1004/trashcan' // Ersetze durch dein Repo (z. B. 'meinname/meinprojekt')
            RELEASE_TAG = "v1.0.${env.BUILD_NUMBER}" // Dynamischer Tag, z. B. v1, v2, ...
        }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
      //   stage('test') {
        //             steps {
          //               sh 'cargo test'
            //         }
              //   }
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
                sh '''
                    # Erstelle ein Release mit einem Tag
                    curl -X POST \
                        -H "Authorization: token $GITHUB_TOKEN" \
                        -H "Accept: application/vnd.github.v3+json" \
                        https://api.github.com/repos/$GITHUB_REPO/releases \
                        -d "{\"tag_name\": \"$RELEASE_TAG\", \"name\": \"Release $RELEASE_TAG\", \"body\": \"Automated release from Jenkins\", \"draft\": false, \"prerelease\": false}"
                '''
            }
        }
        stage('Upload Binary to Release') {
            steps {
                sh '''
                    # Hole die Release-ID
                    RELEASE_ID=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
                        -H "Accept: application/vnd.github.v3+json" \
                        https://api.github.com/repos/$GITHUB_REPO/releases/tags/$RELEASE_TAG | jq -r '.id')

                    # Lade die Binärdatei hoch (z. B. target/release/my_binary)
                    curl -X POST \
                        -H "Authorization: token $GITHUB_TOKEN" \
                        -H "Accept: application/vnd.github.v3+json" \
                        -H "Content-Type: application/octet-stream" \
                        --data-binary @target/release/my_binary \
                        https://uploads.github.com/repos/$GITHUB_REPO/releases/$RELEASE_ID/assets?name=my_binary
                '''
            }
        }

    }
    post {
        always {
            cleanws()
        }
    }

}

