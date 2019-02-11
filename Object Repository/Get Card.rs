<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Card</name>
   <tag></tag>
   <elementGuidId>6a53d0c4-b7d4-4e0d-bd0a-eaab426be57a</elementGuidId>
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>businessTagId</name>
      <type>Main</type>
      <value>${BUSINESS_TAG_ID}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://${BASE_URL}/${API_VERSION}/get/card/${BUSINESS_TAG_ID}/${CARD_ID}?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>70f03f60-8bcd-4ffa-ac7c-8ed79e6fb073</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_VERSION</defaultValue>
      <description></description>
      <id>9b3eb406-3e37-4456-b15b-c9a55d78e7e3</id>
      <masked>false</masked>
      <name>API_VERSION</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BUSINESS_TAG_ID</defaultValue>
      <description></description>
      <id>99dd6352-df46-49f6-9b8b-809870a9ee9d</id>
      <masked>false</masked>
      <name>BUSINESS_TAG_ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.LOGIN_TOKEN</defaultValue>
      <description></description>
      <id>f86592c7-717e-478a-9b1f-25141851d3b8</id>
      <masked>false</masked>
      <name>LOGIN_TOKEN</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>abc5f066-5d92-4d2d-b34d-74a918d123c8</id>
      <masked>false</masked>
      <name>CARD_ID</name>
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
