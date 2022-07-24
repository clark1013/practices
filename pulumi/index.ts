import * as k8s from "@pulumi/kubernetes";
import * as kx from "@pulumi/kubernetesx";
const fs = require('fs')

const kongNS = new k8s.core.v1.Namespace(
    'kong',
    {
      metadata: {
        name: 'kong'
      }
    }
  )

const demoPlugin = new k8s.core.v1.ConfigMap(
'kong-plugin-demo',
{
    metadata: {
    namespace: 'kong',
    name: 'kong-plugin-demo'
    },
    data: {
    'handler.lua': fs.readFileSync('./demo-myheader/handler.lua').toString(),
    'schema.lua': fs.readFileSync('./demo-myheader/schema.lua').toString()
    }
},
{
    dependsOn: [kongNS]
}
)

const kong = new k8s.helm.v3.Chart(
    'kong',
    {
      namespace: 'kong',
      chart: "kong",
      version: "2.8.0",
      fetchOpts:{
        repo: "https://charts.konghq.com",
      },
      values: {
        plugins: {
            configMaps: [
                {
                    name: 'kong-plugin-demo',
                    pluginName: 'demo'
                }
            ]
        },
        proxy: {
            type: 'NodePort'
        }
      }
    },
    {
      dependsOn: [kongNS, demoPlugin]
    }
  )