ARG PORT=5173

FROM node:lts-slim 

WORKDIR /dashboard

COPY package.json package-lock.json .

RUN npm install && npm cache clean --force

COPY . .

EXPOSE $PORT

CMD ["npm", "run", "build"]
