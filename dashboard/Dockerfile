ARG PORT=5173

FROM node:lts-slim 

LABEL org.opencontainers.image.authors="contact@hubblo.org"
LABEL org.opencontainers.image.description="Docker image for the Carenage dashboard, a CI tool for environmental evaluation of software projects"
LABEL org.opencontainers.image.licenses=Apache-2.0
LABEL org.opencontainers.image.title="carenage-dashboard"

WORKDIR /dashboard

COPY package.json package-lock.json .

RUN npm install && npm cache clean --force

COPY . .

EXPOSE $PORT

RUN npm run build

CMD ["node", "build"]
