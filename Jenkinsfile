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

//        stage('Static-Code Analysis') {
//            steps {
//                updateGitlabCommitStatus(name: 'Static-Code Analysis', state: 'pending')
//                withSonarQubeEnv('sonarqube.philipkrauss.it') {
//                    script {
//                        if(env.CI_MERGE_REQUEST_IID) {
//                            sh """
//                                ${SCANNER_HOME}/bin/sonar-scanner \
//                                -Dsonar.pullrequest.key=${env.CI_MERGE_REQUEST_IID} \
//                                -Dsonar.pullrequest.branch=${env.CI_MERGE_REQUEST_SOURCE_BRANCH_NAME} \
//                                -Dsonar.pullrequest.base=${env.CI_MERGE_REQUEST_TARGET_BRANCH_NAME} \
//                                -Dsonar.pullrequest.gitlab.projectId=${env.CI_PROJECT_ID} \
//                                -X
//                            """
//                        } else {
//                            sh """
//                                ${SCANNER_HOME}/bin/sonar-scanner \
//                                -Dsonar.branch.name=${env.BRANCH_NAME} \
//                                -X
//                            """
//                        }
//                    }
//                }
//                timeout(time: 5, unit: 'MINUTES') {
//                    withSonarQubeEnv('sonarqube.philipkrauss.it') {
//                        waitForQualityGate abortPipeline: true
//                    }
//                }
//                updateGitlabCommitStatus(name: 'Static-Code Analysis', state: 'success')
//            }
//        }

        stage('Test') {
            steps {
                sh 'Testing software...'
                sh 'cargo clippy --all-targets --all-features -- -D warnings'
                sh 'cargo test -- --nocapture'
            }
        }

        stage('Build') {
            steps {
                sh 'Building software...'
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
                sh 'Deploying to guthib...'
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
            echo 'Tests and Linting finished.'
        }
        failure {
            echo 'Test or Linting failed, aborting Pipeline'
        }
    }
}
