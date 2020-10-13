# TODO
- [ ] Integrate wait-for-it for smoother deployment
- [ ] Fix and refactor code
- [ ] Normalize responses and provide detailed error responses
- [ ] Refactor code
- [ ] Add unit tests
- Add Github workflow
  - [ ] Simple build
  - [ ] Pushing to a registry
  - [ ] Add end2end tests
  - [ ] Add benchmark tests (siege)
- Add Kubernetes configuration (Kustomize)
  - [ ] ConfigMaps, Secrets
  - [ ] Deployment, Service
  - [ ] Ingress with TLS support and certificate
- Add proper logging
- Consider adding more endpoints to handle:
  - [ ] Tokens management (creation, revoke, expiration)
  - Simple user registration
    - Email confirmation flow
  - Complex user registration flows (OAuth?)
- Consider implementing authorization and privileges
- Monitor the application
- Consider adding more DBMS backends
- [ ] Find a way to optimize docker images size (1GB for dependencies!)