<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Workflow Project</name>
   <tag></tag>
   <elementGuidId>0d32ba8d-aa36-41f6-b10b-2b3851d60217</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Tags\&quot;:\&quot;${WFID}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>jwt</name>
      <type>Main</type>
      <value>${LOGIN_TOKEN}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>businessTagId</name>
      <type>Main</type>
      <value>${BUSINESS_TAG_ID}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://${BASE_URL}/${API_VERSION}/search/${BUSINESS_TAG_ID}/tags/${BUSINESS_TAG_ID}/cards?light=true</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>024bb593-df0d-442f-81f0-7a853a8fc9ea</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_VERSION</defaultValue>
      <description></description>
      <id>19290d7a-8c13-4c64-acd1-64fae902ba6f</id>
      <masked>false</masked>
      <name>API_VERSION</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BUSINESS_TAG_ID</defaultValue>
      <description></description>
      <id>b40cc4aa-c817-4408-8db7-09c146c7a5e9</id>
      <masked>false</masked>
      <name>BUSINESS_TAG_ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.WFID</defaultValue>
      <description></description>
      <id>52a8bddb-6810-4c1c-861f-b4a4c21e5bca</id>
      <masked>false</masked>
      <name>WFID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LOGIN_TOKEN</defaultValue>
      <description></description>
      <id>e0a3ee94-5fd7-424e-a0f0-583df89ade1a</id>
      <masked>false</masked>
      <name>LOGIN_TOKEN</name>
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
