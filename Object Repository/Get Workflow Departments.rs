<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Workflow Departments</name>
   <tag></tag>
   <elementGuidId>8a7febad-5799-4d97-b456-32efd3ffc94a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>jwt</name>
      <type>Main</type>
      <value>${LOGIN_TOKEN}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://${BASE_URL}/${API_VERSION}/${BUSINESS_TAG_ID}/workflows/${WFID}/elements/${TAG}?light=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>f488d53f-8075-4d65-8722-b67cdc62e8ee</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_VERSION</defaultValue>
      <description></description>
      <id>14e40635-d774-4c03-8f47-fe22730e72d5</id>
      <masked>false</masked>
      <name>API_VERSION</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BUSINESS_TAG_ID</defaultValue>
      <description></description>
      <id>f771dfc9-d3c9-42fe-9bfe-ac1c83c9c6bc</id>
      <masked>false</masked>
      <name>BUSINESS_TAG_ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.WFID</defaultValue>
      <description></description>
      <id>6dfef552-b5b6-4206-8643-a69c2f695f77</id>
      <masked>false</masked>
      <name>WFID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LOGIN_TOKEN</defaultValue>
      <description></description>
      <id>52d1495f-eb1c-4fef-a126-76a155a1afcc</id>
      <masked>false</masked>
      <name>LOGIN_TOKEN</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9c1b733c-76bc-49d6-adbe-72cba263f77b</id>
      <masked>false</masked>
      <name>TAG</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
