pipeline {
    agent {
        docker {
            image 'rust:latest'
            //args '--user root -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }

    environment {
        GITHUB_TOKEN = credentials('github-pat') // Jenkins Credential ID for the PAT
        GITHUB_REPO = 'laugenbrezel1004/trashcan' //
        RELEASE_TAG = "v1.0.${env.BUILD_NUMBER}" // Dynamic tag, e.g., v1, v2, ...
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }


stage('Static-Code Analysis') {
            steps {
                updateGitlabCommitStatus(name: 'Static-Code Analysis', state: 'pending')
                withSonarQubeEnv('sonarqube.philipkrauss.it') {
                    script {
                        if(env.CI_MERGE_REQUEST_IID) {
                            sh """
                                ${SCANNER_HOME}/bin/sonar-scanner \
                                -Dsonar.pullrequest.key=${env.CI_MERGE_REQUEST_IID} \
                                -Dsonar.pullrequest.branch=${env.CI_MERGE_REQUEST_SOURCE_BRANCH_NAME} \
                                -Dsonar.pullrequest.base=${env.CI_MERGE_REQUEST_TARGET_BRANCH_NAME} \
                                -Dsonar.pullrequest.gitlab.projectId=${env.CI_PROJECT_ID} \
                                -X
                            """
                        } else {
                            sh """
                                ${SCANNER_HOME}/bin/sonar-scanner \
                                -Dsonar.branch.name=${env.BRANCH_NAME} \
                                -X
                            """
                        }
                    }
                }
                timeout(time: 5, unit: 'MINUTES') {
                    withSonarQubeEnv('sonarqube.philipkrauss.it') {
                        waitForQualityGate abortPipeline: true
                    }
                }
                updateGitlabCommitStatus(name: 'Static-Code Analysis', state: 'success')
            }
            stage('Test') {
                steps {
                    // Parallele Ausführung von Tests und Linting, um Zeit zu sparen
                    parallel(
                        "Unit Tests": {
                            script {
                                // Führe Tests aus und zeige Ausgabe direkt an
                                sh 'cargo test -- --nocapture --test-threads=1'
                                // Optional: JUnit-Bericht generieren (benötigt ein Tool wie cargo2junit)
                                // sh 'cargo test -- -Z unstable-options --format json | cargo2junit > test-report.xml'
                            }
                        },
                        "Linting": {
                            script {
                                // Führe Clippy aus und behandle Warnungen als Fehler
                                sh 'cargo clippy --all-targets --all-features -- -D warnings'
                            }
                        }
                    )
                }
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
          retry(3) {
              createGitHubRelease(
                  credentialId: 'github-pat',
                  repository: 'laugenbrezel1004/trashcan',
                  tag: "${RELEASE_TAG}",
                  commitish: 'main'
              )
          }
      }
  }
    }

    post {
            always {
                // Optional: Testberichte in Jenkins anzeigen (falls JUnit-Bericht generiert wurde)
                // junit 'test-report.xml'
                echo 'Tests and Linting finished.'
            }
            failure {
                echo 'Test or Linting failed, aborting Pipeline'
            }
        }
}
